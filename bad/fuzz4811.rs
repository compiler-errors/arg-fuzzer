
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo4811(_: S3, _: S7, _: S1) {}
        
        fn test4811() { foo4811(S5, S8, S3, S7, S6); }
    