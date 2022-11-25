use heck::SnakeCase;

pub fn ts_config() -> &'static str {
    r#"{
    "compilerOptions": {
      "outDir": "build",
      "target": "es2018",
      "lib": ["es2018", "dom"],
      "module": "commonjs",
      "moduleResolution": "node",
      "declaration": true,
      "checkJs": false,
      "strict": true,
      "noImplicitReturns": true,
      "skipLibCheck": true,
      "allowSyntheticDefaultImports": true,
      "esModuleInterop": true,
      "resolveJsonModule": true,
      "typeRoots": ["./node_modules/@types/", "./types"]
    }
  }
"#
}

pub fn source(name: &str) -> String {
    format!(
        r#"// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

/// This example demonstrates a basic use of a shared object.
/// Rules:
/// - anyone can create and share a counter
/// - everyone can increment a counter by 1
/// - the owner of the counter can reset it to any value
module {}::counter {{
    use sui::transfer;
    use sui::object::{{Self, UID}};
    use sui::tx_context::{{Self, TxContext}};

    /// A shared counter.
    struct Counter has key {{
        id: UID,
        owner: address,
        value: u64
    }}

    public fun owner(counter: &Counter): address {{
        counter.owner
    }}

    public fun value(counter: &Counter): u64 {{
        counter.value
    }}

    /// Create and share a Counter object.
    public entry fun create(ctx: &mut TxContext) {{
        transfer::share_object(Counter {{
            id: object::new(ctx),
            owner: tx_context::sender(ctx),
            value: 0
        }})
    }}

    /// Increment a counter by 1.
    public entry fun increment(counter: &mut Counter) {{
        counter.value = counter.value + 1;
    }}

    /// Set value (only runnable by the Counter owner)
    public entry fun set_value(counter: &mut Counter, value: u64, ctx: &mut TxContext) {{
        assert!(counter.owner == tx_context::sender(ctx), 0);
        counter.value = value;
    }}

    /// Assert a value for the counter.
    public entry fun assert_value(counter: &Counter, value: u64) {{
        assert!(counter.value == value, 0)
    }}
}}
"#,
        name.to_snake_case(),
    )
}

pub fn source_test(name: &str) -> String {
    format!(
        r#"// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

#[test_only]
module {}::counter_test {{
    use sui::test_scenario;
    use {}::counter;

    #[test]
    fun test_counter() {{
        let owner = @0xC0FFEE;
        let user1 = @0xA1;

        let scenario_val = test_scenario::begin(user1);
        let scenario = &mut scenario_val;

        test_scenario::next_tx(scenario, owner);
        {{
            counter::create(test_scenario::ctx(scenario));
        }};

        test_scenario::next_tx(scenario, user1);
        {{
            let counter_val = test_scenario::take_shared<counter::Counter>(scenario);
            let counter = &mut counter_val;

            assert!(counter::owner(counter) == owner, 0);
            assert!(counter::value(counter) == 0, 1);

            counter::increment(counter);
            counter::increment(counter);
            counter::increment(counter);
            test_scenario::return_shared(counter_val);
        }};

        test_scenario::next_tx(scenario, owner);
        {{
            let counter_val = test_scenario::take_shared<counter::Counter>(scenario);
            let counter = &mut counter_val;

            assert!(counter::owner(counter) == owner, 0);
            assert!(counter::value(counter) == 3, 1);

            counter::set_value(counter, 100, test_scenario::ctx(scenario));

            test_scenario::return_shared(counter_val);
        }};

        test_scenario::next_tx(scenario, user1);
        {{
            let counter_val = test_scenario::take_shared<counter::Counter>(scenario);
            let counter = &mut counter_val;

            assert!(counter::owner(counter) == owner, 0);
            assert!(counter::value(counter) == 100, 1);

            counter::increment(counter);

            assert!(counter::value(counter) == 101, 2);

            test_scenario::return_shared(counter_val);
        }};
        test_scenario::end(scenario_val);
    }}
}}"#, name.to_snake_case(), name.to_snake_case())
}

pub fn package_json(name: &str) -> String {
    format!(
        r#"{{
    "name": "{}",
    "version": "1.0.0",
    "description": "",
    "main": "index.js",
    "scripts": {{
        "test": "ts-mocha ./e2etests/**/*.ts"
    }},
    "devDependencies": {{
        "@types/expect": "^24.3.0",
        "@types/mocha": "^10.0.0",
        "mocha": "^10.1.0",
        "ts-mocha": "^10.0.0"
    }},
    "dependencies": {{
        "@mysten/sui.js": "^0.16.0",
        "typescript": "^4.8.4"
    }}
}}"#, name.to_lowercase())
}

pub fn move_manifest(name: &str, test: &str) -> String {
    format!(
        r#"[package]
name = "{}"
version = "0.0.1"

[dependencies]
Sui = {{ git = "https://github.com/MystenLabs/sui.git", subdir = "crates/sui-framework", rev = "devnet" }}

[addresses]
{} =  "0x0"
sui =  "0000000000000000000000000000000000000002"

[ika]
test = "{}"
"#, name.to_snake_case(), name.to_snake_case(), test)
}

pub fn readme() -> &'static str {
    r#"# Starter Project

To run an e2e test all you need to call is

    starter test
"#
}

pub fn ts_test() -> &'static str {
    r#"import {
  JsonRpcProvider,
  Network,
  RawSigner,
  Ed25519Keypair,
  getEvents,
  ObjectId,
  Provider
} from "@mysten/sui.js";
import "mocha";

describe("my_module", () => {
  const provider = new JsonRpcProvider(Network.LOCAL);
  let signers: RawSigner[];
  let creator: RawSigner;
  let player: RawSigner;
  let packageId: string;
  before(async () => {
    signers = createKeystoreSigners(provider);
    creator = signers[0];
    player = signers[1];
  });

  it("deploy contract", async function () {
    packageId = await publishPackage(creator);

    console.log(packageId);

  });
});



export const createKeystoreSigners = (provider: Provider): RawSigner[] => JSON.parse(process.env.SUI_KEYSTORE!).map((key: string) => {
  const toArrayBuffer = (buf: Buffer) => {
    const ab = new ArrayBuffer(buf.length);
    const view = new Uint8Array(ab);
    for (let i = 0; i < buf.length; ++i) {
      view[i] = buf[i];
    }
    return ab;
  }

  const buff = new Uint8Array(
      toArrayBuffer(
          Buffer.from(
              key,
              "base64"
          )
      )
  );
  if(buff.length === 65) {
    const keypair = Ed25519Keypair.fromSeed(buff.slice(33));
    return new RawSigner(keypair, provider);
  } else if(buff.length === 66) {
    throw new Error("The other key type is not supported yet");
  } else {
    throw new Error("Invalid key length");
  }
});

export async function publishPackage(
    signer: RawSigner,
): Promise<ObjectId> {
  const compiledModules = JSON.parse(process.env.SUI_BUILD || "[]");

  const publishTxn = await signer.publish({
    compiledModules: compiledModules,
    gasBudget: 20000,
  });

  const publishEvent = getEvents(publishTxn).filter(
      (e: any) => "publish" in e
  )[0];

  return publishEvent.publish.packageId;
}
"#
}

pub fn gitignore() -> &'static str {
    r#"node_modules
build
.DS_Store
.ika"#
}