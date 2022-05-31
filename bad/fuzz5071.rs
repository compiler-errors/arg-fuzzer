
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo5071(_: S4, _: S3, _: S4, _: S8) {}
        
        fn test5071() { foo5071(S8, S8, S7, S8, S3, S6); }
    