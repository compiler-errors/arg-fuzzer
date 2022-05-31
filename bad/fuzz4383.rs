
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo4383(_: S2, _: S4, _: S5, _: S6, _: S8) {}
        
        fn test4383() { foo4383(S6, S6, S4, S3, S2, S3, S6); }
    