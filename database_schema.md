
## Database Schema

### Seasons Table
```
+----+-------+-----------+-------------+------------+-----------+--------+
| id | name  | is_active | max_players | start_date | stop_date | status |
+----+-------+-----------+-------------+------------+-----------+--------+
```

### MasterRoundTable
```
+----+-----------+--------------+------------+----------+
| id | season_id | round_number | start_time | end_time |
+----+-----------+--------------+------------+----------+
    Foreign Key: season_id -> Seasons(id)
```

### RoundDetailsTable
```
+----+----------+-----------+--------------+----------+--------------+------------+-------------+
| id | round_id | player_id | player_hand  | opponent | opponent_hand| timestamp  | game_status |
+----+----------+-----------+--------------+----------+--------------+------------+-------------+
    Foreign Key: round_id -> MasterRoundTable(id)
```

### PlayerDetailsTable
```
+----+-----------+-----------+----------------+----------------+-------+
| id | season_id | player_id | player_username| player_wallet  | score |
+----+-----------+-----------+----------------+----------------+-------+
    Foreign Key: season_id -> Seasons(id)
```

### ChannelSettings Table
```
+----+---------------------+-------------------+`
| id | broadcast_channel_id| group_channel_id  |
+----+---------------------+-------------------+
```

### MasterCandidateTable
```
+----+-----------+-----------+----------------+----------------+--------------+
| id | season_id | player_id | player_username| player_wallet  | player_status|
+----+-----------+-----------+----------------+----------------+--------------+
    Foreign Key: season_id -> Seasons(id)
```
