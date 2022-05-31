
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo11971(_: S1, _: S4, _: S5, _: S6) {}
        
        fn test11971() { foo11971(S1, S1, S7, S4, S1, S3); }
    