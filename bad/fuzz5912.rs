
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo5912(_: S3, _: S5) {}
        
        fn test5912() { foo5912(S4, S1, S1, S3, S4); }
    