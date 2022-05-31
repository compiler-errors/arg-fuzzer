
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo8291(_: S1, _: S3, _: S5, _: S6, _: S7, _: S8) {}
        
        fn test8291() { foo8291(S3, S2, S1, S0, S5, S6, S6); }
    