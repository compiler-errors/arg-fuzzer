
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo6008(_: S1, _: S6, _: S2) {}
        
        fn test6008() { foo6008(S6, S4, S1, S4, S1); }
    