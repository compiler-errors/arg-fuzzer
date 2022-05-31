
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo204(_: S1, _: S3, _: S5, _: S8, _: S6, _: S7, _: S4, _: S2) {}
        
        fn test204() { foo204(S3, S7, S1, S0, S7); }
    