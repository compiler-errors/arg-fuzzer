
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo5631(_: S7, _: S8) {}
        
        fn test5631() { foo5631(S2, S7, S4, S1, S2); }
    