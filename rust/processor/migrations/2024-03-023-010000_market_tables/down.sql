-- This file should undo anything in `up.sql`

DROP TABLE IF EXISTS market_configs;
DROP TABLE IF EXISTS market_datas;
DROP TABLE IF EXISTS limit_order_datas;
DROP TABLE IF EXISTS open_positions;
DROP TABLE IF EXISTS closed_positions;
DROP TABLE IF EXISTS open_tpsls;
DROP TABLE IF EXISTS closed_tpsls;
DROP TABLE IF EXISTS open_limit_orders;
DROP TABLE IF EXISTS closed_limit_orders;
DROP TABLE IF EXISTS position_datas;
DROP TABLE IF EXISTS tpsl_datas;
DROP VIEW IF EXISTS owner_trades;
DROP TABLE IF EXISTS trade_datas;
DROP TABLE IF EXISTS owner_trades;
DROP TABLE IF EXISTS market_activities;
