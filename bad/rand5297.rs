
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo5297(_: S2, _: S3, _: S4, _: S6, _: S8) {}
        
        fn test5297() { foo5297(S6, S1, S3, S5, S2, S1); }
    