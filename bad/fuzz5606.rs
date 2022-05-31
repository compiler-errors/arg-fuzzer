
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo5606(_: S1, _: S2, _: S3, _: S5) {}
        
        fn test5606() { foo5606(S1, S4, S2, S3, S1); }
    