
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo5503(_: S4, _: S6, _: S3) {}
        
        fn test5503() { foo5503(S1, S4, S5, S6, S7); }
    