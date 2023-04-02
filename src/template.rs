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
}}"#,
        name.to_snake_case(),
        name.to_snake_case()
    )
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
        "@mysten/sui.js": "^0.30.0",
        "typescript": "^4.8.4"
    }}
}}"#,
        name.to_lowercase()
    )
}

pub fn move_manifest(name: &str, test: &str) -> String {
    format!(
        r#"[package]
name = "{}"
version = "0.0.1"

[dependencies]
Sui = {{ git = "https://github.com/MystenLabs/sui.git", subdir = "crates/sui-framework/packages/sui-framework", rev = "testnet" }}

[addresses]
{} =  "0x0"
sui =  "0000000000000000000000000000000000000000000000000000000000000002"

[ika]
test = "{}"
"#,
        name.to_snake_case(),
        name.to_snake_case(),
        test
    )
}

pub fn readme(name: &str) -> String {
    format!(
        r#"# {}

To run move test and e2e test:

    ika test

To run only the move test:

    ika test --skip-e2e

To run only the e2e test:

    ika test --skip-contract

To run the e2e with a clean ledger:

    ika test --clear
"#,
        name.to_snake_case()
    )
}

pub fn ts_test() -> &'static str {
    r#"import {
  JsonRpcProvider,
  RawSigner,
  Ed25519Keypair,
  ObjectId,
  TransactionBlock,
  fromB64,
  normalizeSuiObjectId,
  Connection,
  localnetConnection,
} from "@mysten/sui.js";
import "mocha";

describe("my_module", () => {
  const provider = new JsonRpcProvider(new Connection(localnetConnection));
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
    packageId = await publishPackage(provider, creator);

    console.log(packageId);
  });
});

export const createKeystoreSigners = (provider: JsonRpcProvider): RawSigner[] =>
  JSON.parse(process.env.SUI_KEYSTORE!).map((key: string) => {
    const toArrayBuffer = (buf: Buffer) => {
      const ab = new ArrayBuffer(buf.length);
      const view = new Uint8Array(ab);
      for (let i = 0; i < buf.length; ++i) {
        view[i] = buf[i];
      }
      return ab;
    };

    const buff = new Uint8Array(toArrayBuffer(Buffer.from(key, "base64")));
    const keypair = Ed25519Keypair.fromSecretKey(buff.slice(1));
    return new RawSigner(keypair, provider);
  });

export async function publishPackage(
  provider: JsonRpcProvider,
  signer: RawSigner
): Promise<ObjectId> {
  const compiledModules = JSON.parse(process.env.SUI_BUILD || "[]");
  const tx = new TransactionBlock();
  const [upgradeCap] = tx.publish(
    compiledModules.modules.map((m: any) => Array.from(fromB64(m))),
    compiledModules.dependencies.map((addr: string) =>
      normalizeSuiObjectId(addr)
    )
  );
  tx.transferObjects([upgradeCap], tx.pure(await signer.getAddress()));
  const result = await signer.signAndExecuteTransactionBlock({
    transactionBlock: tx,
  });
  const txn = await provider.getTransactionBlock({
    digest: result.digest,
    options: {
      showObjectChanges: true,
    },
  });
  const publishEvent = txn.objectChanges?.filter(
    (e) => e.type === "published"
  )[0] as {
    packageId: string;
    type: "published";
    version: number;
    digest: string;
    modules: string[];
  };
  return publishEvent!.packageId;
}
"#
}

pub fn gitignore() -> &'static str {
    r#"node_modules
build
.DS_Store
.ika"#
}
