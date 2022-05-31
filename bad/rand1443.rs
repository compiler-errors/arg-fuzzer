
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo1443(_: S4, _: S2, _: S0, _: S6, _: S7, _: S7) {}
        
        fn test1443() { foo1443(S5, S6, S7, S4, S4, S3, S0); }
    