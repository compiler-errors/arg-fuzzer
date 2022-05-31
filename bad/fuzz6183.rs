
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo6183(_: S2, _: S3, _: S6) {}
        
        fn test6183() { foo6183(S7, S8, S4); }
    