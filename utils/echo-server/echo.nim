import asyncnet, asyncdispatch
 
proc processClient(client: AsyncSocket) {.async.} =
  while true:
    let line = await client.recvLine()
    echo line
    await client.send(line & "\c\L")
 
proc serve() {.async.} =
  var server = newAsyncSocket()
  server.bindAddr(Port(12345))
  server.listen()
 
  while true:
    let client = await server.accept()
    echo "new client ...."
    discard processClient(client)
 
discard serve()
runForever()