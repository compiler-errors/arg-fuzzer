
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo8953(_: S3, _: S4, _: S6) {}
        
        fn test8953() { foo8953(S5, S6, S0, S3, S1); }
    