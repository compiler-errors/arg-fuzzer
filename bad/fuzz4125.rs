
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo4125(_: S4, _: S2, _: S8, _: S6, _: S8) {}
        
        fn test4125() { foo4125(S3, S5, S4, S6, S3, S3, S6); }
    