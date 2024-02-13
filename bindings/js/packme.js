export const init = async (buffer) => {
  const module = await WebAssembly.instantiate(buffer, {});

  const alloc = module.instance.exports.alloc;
  const dealloc = module.instance.exports.dealloc;
  const _pack = module.instance.exports.pack;

  const pack = (data) => {
    const [ptr, len] = getStringPtr(JSON.stringify(data));
    const resultPtr = _pack(ptr, len);
    const [result, resultLen] = parseString(resultPtr);
    dealloc(ptr, len);
    dealloc(resultPtr, resultLen);
    return JSON.parse(result);
  }

  const getStringPtr = (strData) => {
    const encoder = new TextEncoder();
    const str = encoder.encode(strData);
    const ptr = alloc(str.length);
    const mem = new Uint8Array(module.instance.exports.memory.buffer);
    mem.set(str, ptr);
    return [ptr, str.length];
  }

  const parseString = (ptr) => {
    const mem = new Uint8Array(module.instance.exports.memory.buffer, ptr);
    let str = '';
    let len = 0;
    while (mem[len] !== 0) {
      str += String.fromCharCode(mem[len]);
      len++;
    }
    return [str, len];
  }

  return {
    pack
  };
};