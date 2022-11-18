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
        r#"module {}::my_module {{
    use sui::object::{{Self, UID}};
    use sui::transfer;
    use sui::tx_context::{{Self, TxContext}};

    struct Forge has key, store {{
        id: UID,
        swords_created: u64,
    }}

    fun init(ctx: &mut TxContext) {{
        let admin = Forge {{
            id: object::new(ctx),
            swords_created: 0,
        }};
        // transfer the forge object to the module/package publisher
        transfer::transfer(admin, tx_context::sender(ctx));
    }}

    public fun swords_created(self: &Forge): u64 {{
        self.swords_created
    }}


    #[test]
    fun test_sword_transactions() {{
        use sui::test_scenario;

        // create test addresses representing users
        let admin = @0xBABE;
        let initial_owner = @0xCAFE;
        let final_owner = @0xFACE;

        // first transaction to emulate module initialization
        let scenario_val = test_scenario::begin(admin);
        let scenario = &mut scenario_val;
        {{
            init(test_scenario::ctx(scenario));
        }};
        test_scenario::end(scenario_val);
    }}
}}
"#,
        name.to_snake_case(),
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

[extra]
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