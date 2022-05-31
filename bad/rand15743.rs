
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo15743(_: S1, _: S3, _: S8) {}
        
        fn test15743() { foo15743(S2, S1, S5, S5, S7, S7, S1); }
    