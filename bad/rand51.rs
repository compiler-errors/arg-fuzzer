
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo51(_: S1, _: S2, _: S3) {}
        
        fn test51() { foo51(S6, S6, S6, S2, S5, S6); }
    