export const readData = async (key:string): Promise<string> => {
    const resp = await fetch("http://localhost:8080/sync/" + key)
    return resp.text()
}