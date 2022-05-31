
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo5229(_: S3, _: S4, _: S7) {}
        
        fn test5229() { foo5229(S4, S1, S3, S6, S8); }
    