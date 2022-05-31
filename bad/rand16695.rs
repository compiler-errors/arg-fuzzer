
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo16695(_: S2, _: S3, _: S4, _: S7) {}
        
        fn test16695() { foo16695(S1, S4, S2, S1, S4, S5, S3); }
    