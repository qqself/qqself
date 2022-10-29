// TODO Key file is a nice abstraction and could be reused in CLI as well
//      It should be part of the core

export interface Keys {
  publicKey: string;
  privateKey: string;
}

// Serialize key file to string
export const keysToString = (keys: Keys): string => {
  return `${keys.publicKey}\n${keys.privateKey}`;
};

// Deserialize keys from string
export const stringToKeys = (s: string): Keys | Error => {
  if (!s.length) return new Error("Key file is empty");
  const lines = s.split("\n");
  if (lines.length != 2 || !lines[0].trim().length || !lines[1].trim().length)
    return new Error(
      "Key file should have two lines with public and private keys"
    );
  return {
    publicKey: lines[0],
    privateKey: lines[1],
  };
};
