
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo5617(_: S3, _: S6) {}
        
        fn test5617() { foo5617(S1, S3, S4, S5, S7, S8); }
    