# universal_automation

## Start server: 
Run Command:  'cargo run' to start server. 

## Testing with postman:
1. new gRPC request
2. localhost:50051
3. add the ../proto/automation.proto script
4. select the automationService / SendCommand
5. add the message : { "command": "test-command" }
