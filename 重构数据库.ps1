# �������ݿ�
pg_ctl start

# �ȴ����ݿ�����
Start-Sleep -Seconds 3

# ɾ�����ݿ�
psql -U postgres -d postgres -c "DROP DATABASE IF EXISTS shorekeeper;"

# �½����ݿ�
psql -U postgres -d postgres -c "CREATE DATABASE shorekeeper;"

# ά������
while ($true) {
    Start-Sleep -Seconds 3600
}
