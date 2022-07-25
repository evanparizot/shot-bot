# shot-bot


## Deploy Manually
1. Start docker. Run `aws-build al2`
2. Change to `/shot-bot/target`
3. Run the following (kill process on host first)

``` bash
scp -i ~/shot-bot.pem latest-al2 ec2-user@ec2-3-17-156-95.us-east-2.compute.amazonaws.com:/home/ec2-user/
```

4. SSH to host
5. Run `nohup ./latest-al2 &`