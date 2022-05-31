
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo6153(_: S0, _: S3, _: S5, _: S6) {}
        
        fn test6153() { foo6153(S3, S4, S3, S7, S4, S7); }
    