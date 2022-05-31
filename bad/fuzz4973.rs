
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo4973(_: S1, _: S3) {}
        
        fn test4973() { foo4973(S7, S1, S2, S4, S6); }
    