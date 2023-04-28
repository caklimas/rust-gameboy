import { RustGameboy } from "../redux/state/rustGameboy";

let wasm: RustGameboy | null = null;
export async function loadWasm(): Promise<RustGameboy | undefined> {
  debugger;
  if (!!wasm) {
    return wasm;
  }

  try {
    const loadedWasm = await import("../../pkg");
    wasm = loadedWasm;
    return wasm;
  } catch (err) {
    console.error(
      `Unexpected error in loadWasm. [Message: ${(err as any).message}]`
    );
  }
}
