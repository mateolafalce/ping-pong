mod prelude;
use prelude::*;
    // The function receives a Player object and a board represented by a vector of vectors of chars, and returns the modified board.
    #[test]
    fn test_player_positioning() {
        
        
        // Initialize player and board
        let player = Player {
            x: 0,
            y: 0,
            width: 2,
            speed: 1,
            height: 2
        };
        let board: Vec<Vec<char>> = vec![
            vec!['-', '-'],
            vec!['-', '-'],
        ];
        
        // Call the code under test
        let modified_board = position_player(player, board);
        
        // Assert the modified board
        assert_eq!(modified_board, vec![
            vec!['|', '|'],
            vec!['|', '|'],
        ]);
    }
    
        // The function iterates over the board and modifies the positions where the player is located.
    #[test]
    fn test_player_iteration() {
        
        // Initialize player and board
        let player = Player {
            x: 1,
            y: 1,
            width: 2,
            speed: 1,
            height: 2,
        };
        let board = vec![
            vec!['-', '-'],
            vec!['-', '-'],
        ];
        
        // Call the code under test
        let modified_board = position_player(player, board);
        
        // Assert the modified board
        assert_eq!(modified_board, vec![
            vec!['-', '-'],
            vec!['-', '|'],
        ]);
    }
    
        // The function correctly modifies the board when the player is located at the top or left edge of the board.
    #[test]
    fn test_player_top_left_edge() {
        
        
        // Initialize player and board
        let player = Player {
            x: 0,
            y: 0,
            speed:1,
            width: 2,
            height: 2,
        };
        let board = vec![
            vec!['-', '-'],
            vec!['-', '-'],
        ];
        
        // Call the code under test
        let modified_board = position_player(player, board);
        
        // Assert the modified board
        assert_eq!(modified_board, vec![
            vec!['|', '|'],
            vec!['|', '|'],
        ]);
    }
    
        // The function correctly modifies the board when the player is located at position (0,0).
    #[test]
    fn test_player_position_0_0() {
        
        
        // Initialize player and board
        let player = Player {
            x: 0,
            y: 0,
            speed:1,
            width: 1,
            height: 1,
        };
        let board = vec![
            vec!['-'],
        ];
        
        // Call the code under test
        let modified_board = position_player(player, board);
        
        // Assert the modified board
        assert_eq!(modified_board, vec![
            vec!['|'],
        ]);
    }
    
        // The function correctly modifies the board when the player is located at position (lines-1, columns-1).
    #[test]
    fn test_player_position_lines_columns() {
        
        
        // Initialize player and board
        let player = Player {
            x: 1,
            y: 1,
            width: 1,
            speed: 1,
            height: 1,
        };
        let board = vec![
            vec!['-', '-'],
            vec!['-', '-'],
        ];
        
        // Call the code under test
        let modified_board = position_player(player, board);
        
        // Assert the modified board
        assert_eq!(modified_board, vec![
            vec!['-', '-'],
            vec!['-', '|'],
        ]);
    }
    
        // The function correctly modifies the board when the player is located at position (0, columns-1).
    #[test]
    fn test_player_position_0_columns() {
        
        
        // Initialize player and board
        let player = Player {
            x: 1,
            y: 0,
            speed:1,
            width: 1,
            height: 1,
        };
        let board = vec![
            vec!['-', '-'],
            vec!['-', '-'],
        ];
        
        // Call the code under test
        let modified_board = position_player(player, board);
        
        // Assert the modified board
        assert_eq!(modified_board, vec![
            vec!['-', '|'],
            vec!['-', '-'],
        ]);
    }
    