
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo3975(_: S4, _: S6, _: S5, _: S8, _: S7, _: S3, _: S1, _: S2) {}
        
        fn test3975() { foo3975(S7, S1, S3, S0, S1, S4); }
    