-- phpMyAdmin SQL Dump
-- version 5.0.2
-- https://www.phpmyadmin.net/
--
-- ホスト: db
-- 生成日時: 2023 年 7 月 18 日 13:54
-- サーバのバージョン： 8.0.27
-- PHP のバージョン: 7.4.11

SET SQL_MODE = "NO_AUTO_VALUE_ON_ZERO";
START TRANSACTION;
SET time_zone = "+00:00";


/*!40101 SET @OLD_CHARACTER_SET_CLIENT=@@CHARACTER_SET_CLIENT */;
/*!40101 SET @OLD_CHARACTER_SET_RESULTS=@@CHARACTER_SET_RESULTS */;
/*!40101 SET @OLD_COLLATION_CONNECTION=@@COLLATION_CONNECTION */;
/*!40101 SET NAMES utf8mb4 */;

--
-- データベース: `suzuki`
--

-- --------------------------------------------------------

--
-- テーブルの構造 `t_user`
--

CREATE TABLE `t_user` (
  `id` int UNSIGNED NOT NULL,
  `email` varchar(256) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL,
  `created_at` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP,
  `updated_at` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

--
-- テーブルのデータのダンプ `t_user`
--

INSERT INTO `t_user` (`id`, `email`, `created_at`) VALUES
(1, 'test1@example.com', '2023-07-18 21:47:33'),
(2, 'test2@example.com', '2023-07-18 21:47:33'),
(3, 'test3@example.com', '2023-07-18 21:47:33'),
(4, 'test4@example.com', '2023-07-18 21:47:33'),
(5, 'test5@example.com', '2023-07-18 21:47:33'),
(6, 'test6@example.com', '2023-07-18 21:47:33'),
(7, 'test7@example.com', '2023-07-18 21:47:33'),
(8, 'test8@example.com', '2023-07-18 21:47:33'),
(9, 'test9@example.com', '2023-07-18 21:47:33'),
(10, 'test10@example.com', '2023-07-18 21:47:33');

-- --------------------------------------------------------

--
-- テーブルの構造 `t_user_slot`
--

CREATE TABLE `t_user_slot` (
  `id` int UNSIGNED NOT NULL,
  `user_id` int UNSIGNED NOT NULL,
  `start` datetime NOT NULL,
  `created_at` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP,
  `updated_at` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

--
-- ダンプしたテーブルのインデックス
--

--
-- テーブルのインデックス `t_user`
--
ALTER TABLE `t_user`
  ADD PRIMARY KEY (`id`),
  ADD UNIQUE KEY `email` (`email`);

--
-- テーブルのインデックス `t_user_slot`
--
ALTER TABLE `t_user_slot`
  ADD PRIMARY KEY (`id`),
  ADD UNIQUE KEY `user_id` (`user_id`,`start`);

--
-- ダンプしたテーブルのAUTO_INCREMENT
--

--
-- テーブルのAUTO_INCREMENT `t_user`
--
ALTER TABLE `t_user`
  MODIFY `id` int UNSIGNED NOT NULL AUTO_INCREMENT, AUTO_INCREMENT=11;

--
-- テーブルのAUTO_INCREMENT `t_user_slot`
--
ALTER TABLE `t_user_slot`
  MODIFY `id` int UNSIGNED NOT NULL AUTO_INCREMENT;

--
-- ダンプしたテーブルの制約
--

--
-- テーブルの制約 `t_user_slot`
--
ALTER TABLE `t_user_slot`
  ADD CONSTRAINT `t_user_slot_ibfk_1` FOREIGN KEY (`user_id`) REFERENCES `t_user` (`id`) ON DELETE RESTRICT ON UPDATE RESTRICT;
COMMIT;

/*!40101 SET CHARACTER_SET_CLIENT=@OLD_CHARACTER_SET_CLIENT */;
/*!40101 SET CHARACTER_SET_RESULTS=@OLD_CHARACTER_SET_RESULTS */;
/*!40101 SET COLLATION_CONNECTION=@OLD_COLLATION_CONNECTION */;
