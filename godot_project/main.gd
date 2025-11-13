extends Node

func _on_power_up_collected():
	var player_node = $Player
	player_node.increase_speed(200.0)
