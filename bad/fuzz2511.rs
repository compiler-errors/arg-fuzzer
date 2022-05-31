
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo2511(_: S4, _: S5) {}
        
        fn test2511() { foo2511(S1, S3, S4, S7, S8); }
    