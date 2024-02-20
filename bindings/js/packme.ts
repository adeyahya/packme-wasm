import { AlgoInput, AlgoResult } from "./interface";
import { msg, __wbg_set_wasm } from "./bg";

export const init = async (buffer: ArrayBuffer) => {
  const module = await WebAssembly.instantiate(buffer, {});
  __wbg_set_wasm(module.instance.exports);

  const pack = (input: AlgoInput): AlgoResult => {
    const result = msg(JSON.stringify({ Pack: { input } }));
    return JSON.parse(result) as AlgoResult;
  };

  const version = (): string => {
    const result = msg(JSON.stringify({ Version: {} }));
    return result;
  };

  return {
    pack,
    version,
  };
};
