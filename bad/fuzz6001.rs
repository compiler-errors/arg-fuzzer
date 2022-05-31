
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo6001(_: S4, _: S5) {}
        
        fn test6001() { foo6001(S6, S4, S7); }
    