
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo1050(_: S7, _: S6, _: S1, _: S6, _: S0, _: S7) {}
        
        fn test1050() { foo1050(S1, S2, S3, S4, S5, S6, S8); }
    