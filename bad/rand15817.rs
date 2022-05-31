
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo15817(_: S5, _: S3, _: S1, _: S0, _: S7) {}
        
        fn test15817() { foo15817(S4, S2, S3, S6, S1, S7, S8, S5); }
    