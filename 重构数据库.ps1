# 启动数据库
pg_ctl start

# 等待数据库启动
Start-Sleep -Seconds 3

# 删除数据库
psql -U postgres -d postgres -c "DROP DATABASE IF EXISTS shorekeeper;"

# 新建数据库
psql -U postgres -d postgres -c "CREATE DATABASE shorekeeper;"

# 维持运行
while ($true) {
    Start-Sleep -Seconds 3600
}
