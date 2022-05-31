
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo2509(_: S1, _: S2, _: S3, _: S8) {}
        
        fn test2509() { foo2509(S3, S4, S2, S6, S5, S6, S1, S7); }
    