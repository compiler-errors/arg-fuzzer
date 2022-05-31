
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo6855(_: S8, _: S4, _: S5, _: S2, _: S3) {}
        
        fn test6855() { foo6855(S7, S1, S7, S3, S6, S1, S4); }
    