
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo6105(_: S2, _: S7, _: S5, _: S3) {}
        
        fn test6105() { foo6105(S1, S2, S4, S5, S6, S7, S8); }
    