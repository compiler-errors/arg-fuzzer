
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo4035(_: S1, _: S4, _: S5) {}
        
        fn test4035() { foo4035(S6, S3, S3, S4, S8, S4, S8, S6); }
    