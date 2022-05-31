
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo5689(_: S1, _: S3, _: S6, _: S4) {}
        
        fn test5689() { foo5689(S7, S2, S2, S7, S3, S7); }
    