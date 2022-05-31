
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo16873(_: S1, _: S3, _: S5, _: S0, _: S0, _: S0) {}
        
        fn test16873() { foo16873(S1, S2, S3, S5, S6, S7, S8); }
    