
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo8889(_: S1, _: S3, _: S4, _: S5, _: S8) {}
        
        fn test8889() { foo8889(S1, S6, S3, S0, S7, S5); }
    