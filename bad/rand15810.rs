
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo15810(_: S5, _: S6) {}
        
        fn test15810() { foo15810(S1, S3, S5, S8); }
    