
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo5520(_: S2, _: S3, _: S6, _: S3, _: S8) {}
        
        fn test5520() { foo5520(S7, S7, S3, S3, S5, S6, S7, S6); }
    