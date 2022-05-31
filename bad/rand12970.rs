
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo12970(_: S5, _: S3) {}
        
        fn test12970() { foo12970(S1, S5, S1, S1, S5); }
    