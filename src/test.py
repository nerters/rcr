import redis # type: ignore

# 连接到 Redis
r = redis.StrictRedis(host='localhost', port=6379, db=0)

# 订阅键的事件
pubsub = r.pubsub()
pubsub.psubscribe('__keyevent@0__:set', '__keyevent@0__:del')

# 监听事件
for message in pubsub.listen():
    print(message)
