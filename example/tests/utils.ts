import { Actor, HttpAgent } from "@dfinity/agent";
import { _SERVICE as CandyFunctionsCanister } from "dfx-gen/candyFunctions/candyFunctions.did";
import { _SERVICE as WorkspaceCanister } from "dfx-gen/workspace/workspace.did";
import { _SERVICE as RustCanister } from "dfx-gen/rust/rust.did";
import canister_ids from "../.dfx/local/canister_ids.json";
import { idlFactory as candyFunctionsFactory } from "../src/declarations/candyFunctions/index";
import { idlFactory as workspaceFactory } from "../src/declarations/workspace/index";
import { idlFactory as rustIDLFactory } from "../src/declarations/rust/index";

export const createRustActor = (): RustCanister => {
  const agent = new HttpAgent({
    host: "http://localhost:8005",
  });
  agent.fetchRootKey();
  const actor = Actor.createActor<RustCanister>(rustIDLFactory, {
    agent,
    canisterId: canister_ids.rust.local,
  });

  return actor;
};

export const createMotokoActor = (): CandyFunctionsCanister => {
  const agent = new HttpAgent({
    host: "http://localhost:8005",
  });
  agent.fetchRootKey();
  const actor = Actor.createActor<CandyFunctionsCanister>(
      candyFunctionsFactory,
      {
        agent,
        canisterId: canister_ids.candyFunctions.local,
      }
  );

  return actor;
};

export const workspaceActor = (): WorkspaceCanister => {
  const agent = new HttpAgent({
    host: "http://localhost:8005",
  });
  agent.fetchRootKey();
  const actor = Actor.createActor<WorkspaceCanister>(workspaceFactory, {
    agent,
    canisterId: canister_ids.workspace.local,
  });

  return actor;
};
